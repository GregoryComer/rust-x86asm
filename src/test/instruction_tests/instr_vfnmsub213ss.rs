use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 175, 247], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1299317455, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 175, 153, 207, 2, 114, 77], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 175, 247], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 1093481498, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 175, 151, 26, 52, 45, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 189, 175, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1244969805, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 139, 175, 4, 85, 77, 187, 52, 74], OperandSize::Dword)
}

#[test]
fn vfnmsub213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 61, 159, 175, 245], OperandSize::Qword)
}

#[test]
fn vfnmsub213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RAX, 127562866, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 61, 143, 175, 168, 114, 116, 154, 7], OperandSize::Qword)
}

