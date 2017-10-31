use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 97, 226], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 97, 20, 210], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 97, 240], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RBX, 1827914992, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 97, 171, 240, 196, 243, 108], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 97, 207], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 311981293, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 97, 44, 253, 237, 116, 152, 18], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 97, 219], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 2034907578, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 97, 129, 186, 57, 74, 121], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 97, 198], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 587255719, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 97, 129, 167, 207, 0, 35], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 45, 139, 97, 244], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 258965445, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 117, 142, 97, 156, 82, 197, 127, 111, 15], OperandSize::Qword)
}

