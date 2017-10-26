use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 175, 235], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 175, 7], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 175, 250], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 175, 28, 254], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 218, 175, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 389097744, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 175, 143, 16, 41, 49, 23], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 5, 150, 175, 235], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RSI, 1840148012, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 134, 175, 158, 44, 110, 174, 109], OperandSize::Qword)
}

