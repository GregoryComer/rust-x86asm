use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 210], OperandSize::Dword)
}

#[test]
fn vmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1422648044, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 161, 236, 226, 203, 84], OperandSize::Dword)
}

#[test]
fn vmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 212], OperandSize::Qword)
}

#[test]
fn vmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 43], OperandSize::Qword)
}

#[test]
fn vmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 241], OperandSize::Dword)
}

#[test]
fn vmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1171854689, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 164, 194, 97, 21, 217, 69], OperandSize::Dword)
}

#[test]
fn vmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 8, 110, 237], OperandSize::Qword)
}

#[test]
fn vmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 28, 242], OperandSize::Qword)
}

#[test]
fn vmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 228], OperandSize::Dword)
}

#[test]
fn vmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1263608004, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 180, 255, 196, 32, 81, 75], OperandSize::Dword)
}

#[test]
fn vmovd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 243], OperandSize::Qword)
}

#[test]
fn vmovd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 356124555, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 188, 200, 139, 7, 58, 21], OperandSize::Qword)
}

#[test]
fn vmovd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 225], OperandSize::Dword)
}

#[test]
fn vmovd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(EBX, 1549082000, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 163, 144, 29, 85, 92], OperandSize::Dword)
}

#[test]
fn vmovd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 126, 212], OperandSize::Qword)
}

#[test]
fn vmovd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 30], OperandSize::Qword)
}

