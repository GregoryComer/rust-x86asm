use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 42, 196], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 1862230524, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 42, 184, 252, 97, 255, 110], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 42, 255], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 42, 20, 211], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 194, 42, 195], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1467576980, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 194, 42, 36, 149, 148, 114, 121, 87], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 86, 24, 42, 231], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 685210251, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 42, 28, 93, 139, 122, 215, 40], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 46, 80, 42, 238], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 46, 8, 42, 62], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 206, 24, 42, 205], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1141279185, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 246, 8, 42, 140, 191, 209, 137, 6, 68], OperandSize::Qword)
}

