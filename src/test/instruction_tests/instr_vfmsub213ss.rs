use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 171, 195], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 171, 59], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 171, 254], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 912513807, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 171, 172, 254, 15, 219, 99, 54], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 189, 171, 204], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1132410721, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 171, 187, 97, 55, 127, 67], OperandSize::Dword)
}

#[test]
fn vfmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 150, 171, 227], OperandSize::Qword)
}

#[test]
fn vfmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RCX, 851224790, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 85, 130, 171, 185, 214, 168, 188, 50], OperandSize::Qword)
}

