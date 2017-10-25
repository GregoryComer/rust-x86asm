use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 24], OperandSize::Word)
}

#[test]
fn movbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 36, 247], OperandSize::Dword)
}

#[test]
fn movbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 12, 151], OperandSize::Qword)
}

#[test]
fn movbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BP, 238, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 158, 238, 0], OperandSize::Word)
}

#[test]
fn movbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1931697010, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 156, 131, 114, 91, 35, 115], OperandSize::Dword)
}

#[test]
fn movbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 60, 79], OperandSize::Qword)
}

#[test]
fn movbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 240, 44, 114], OperandSize::Qword)
}

#[test]
fn movbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 21], OperandSize::Word)
}

#[test]
fn movbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 59], OperandSize::Dword)
}

#[test]
fn movbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 28, 129], OperandSize::Qword)
}

#[test]
fn movbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 31266, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 154, 34, 122], OperandSize::Word)
}

#[test]
fn movbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 2011002547, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 36, 221, 179, 118, 221, 119], OperandSize::Dword)
}

#[test]
fn movbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 2003934524, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 180, 120, 60, 157, 113, 119], OperandSize::Qword)
}

#[test]
fn movbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 241, 19], OperandSize::Qword)
}

