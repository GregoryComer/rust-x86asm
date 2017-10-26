use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn popcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 217], OperandSize::Word)
}

#[test]
fn popcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 25638, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 144, 38, 100], OperandSize::Word)
}

#[test]
fn popcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 229], OperandSize::Dword)
}

#[test]
fn popcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 844211472, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 20, 181, 16, 165, 81, 50], OperandSize::Dword)
}

#[test]
fn popcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 223], OperandSize::Qword)
}

#[test]
fn popcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1037636549, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 44, 141, 197, 19, 217, 61], OperandSize::Qword)
}

#[test]
fn popcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 217], OperandSize::Word)
}

#[test]
fn popcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 66, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 91, 66], OperandSize::Word)
}

#[test]
fn popcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 254], OperandSize::Dword)
}

#[test]
fn popcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(ESI, 584881381, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 142, 229, 148, 220, 34], OperandSize::Dword)
}

#[test]
fn popcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 230], OperandSize::Qword)
}

#[test]
fn popcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 25], OperandSize::Qword)
}

#[test]
fn popcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 234], OperandSize::Qword)
}

#[test]
fn popcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 60, 154], OperandSize::Qword)
}

