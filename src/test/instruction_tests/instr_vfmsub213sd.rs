use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 171, 192], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 171, 31], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 171, 245], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 278502365, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 171, 12, 253, 221, 155, 153, 16], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 253, 171, 214], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 171, 59], OperandSize::Dword)
}

#[test]
fn vfmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 189, 211, 171, 197], OperandSize::Qword)
}

#[test]
fn vfmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 900797823, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 149, 134, 171, 148, 190, 127, 21, 177, 53], OperandSize::Qword)
}

