use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 218, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 146, 218, 0], OperandSize::Word)
}

#[test]
fn movbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(BP)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 42], OperandSize::Dword)
}

#[test]
fn movbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 518400876, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 52, 221, 108, 43, 230, 30], OperandSize::Qword)
}

#[test]
fn movbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 21312, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 177, 64, 83], OperandSize::Word)
}

#[test]
fn movbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1646305452, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 172, 192, 172, 160, 32, 98], OperandSize::Dword)
}

#[test]
fn movbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RDX, 933068572, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 186, 28, 127, 157, 55], OperandSize::Qword)
}

#[test]
fn movbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 240, 50], OperandSize::Qword)
}

#[test]
fn movbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 238, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 169, 238, 0], OperandSize::Word)
}

#[test]
fn movbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 59], OperandSize::Dword)
}

#[test]
fn movbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 11], OperandSize::Qword)
}

#[test]
fn movbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectDisplaced(SI, 5186, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 164, 66, 20], OperandSize::Word)
}

#[test]
fn movbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectDisplaced(EBX, 791461777, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 163, 145, 191, 44, 47], OperandSize::Dword)
}

#[test]
fn movbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledDisplaced(RDI, Two, 614368731, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 36, 125, 219, 133, 158, 36], OperandSize::Qword)
}

#[test]
fn movbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectDisplaced(RCX, 1643220830, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 241, 177, 94, 143, 241, 97], OperandSize::Qword)
}

