use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM3)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 221], OperandSize::Dword)
}

#[test]
fn movd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 26], OperandSize::Dword)
}

#[test]
fn movd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM5)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 238], OperandSize::Qword)
}

#[test]
fn movd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1349100547, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 52, 205, 3, 164, 105, 80], OperandSize::Qword)
}

#[test]
fn movd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 250], OperandSize::Dword)
}

#[test]
fn movd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 60, 81], OperandSize::Dword)
}

#[test]
fn movd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 195], OperandSize::Qword)
}

#[test]
fn movd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 598470386, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 36, 205, 242, 238, 171, 35], OperandSize::Qword)
}

#[test]
fn movd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 253], OperandSize::Dword)
}

#[test]
fn movd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(EAX, 1169462826, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 176, 42, 150, 180, 69], OperandSize::Dword)
}

#[test]
fn movd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 243], OperandSize::Qword)
}

#[test]
fn movd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1403067518, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 4, 133, 126, 28, 161, 83], OperandSize::Qword)
}

#[test]
fn movd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 204], OperandSize::Dword)
}

#[test]
fn movd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 44, 186], OperandSize::Dword)
}

#[test]
fn movd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 229], OperandSize::Qword)
}

#[test]
fn movd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(RBX, 306924691, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 139, 147, 76, 75, 18], OperandSize::Qword)
}

