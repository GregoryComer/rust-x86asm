use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 171, 242], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 171, 20, 250], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 171, 194], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 633681688, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 171, 44, 181, 24, 55, 197, 37], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 185, 171, 200], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 171, 35], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 253, 252, 171, 206], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 205, 132, 171, 9], OperandSize::Qword)
}

