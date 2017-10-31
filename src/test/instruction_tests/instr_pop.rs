use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pop_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ES)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[7], OperandSize::Word)
}

#[test]
fn pop_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ES)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[7], OperandSize::Dword)
}

#[test]
fn pop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(SS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[23], OperandSize::Word)
}

#[test]
fn pop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(SS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[23], OperandSize::Dword)
}

#[test]
fn pop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[31], OperandSize::Word)
}

#[test]
fn pop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[31], OperandSize::Dword)
}

#[test]
fn pop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[91], OperandSize::Word)
}

#[test]
fn pop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 90], OperandSize::Dword)
}

#[test]
fn pop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 93], OperandSize::Qword)
}

#[test]
fn pop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 90], OperandSize::Word)
}

#[test]
fn pop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[90], OperandSize::Dword)
}

#[test]
fn pop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[94], OperandSize::Qword)
}

#[test]
fn pop_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[95], OperandSize::Word)
}

#[test]
fn pop_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[143, 1], OperandSize::Word)
}

#[test]
fn pop_15() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 93], OperandSize::Dword)
}

#[test]
fn pop_16() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 143, 4, 94], OperandSize::Dword)
}

#[test]
fn pop_17() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 93], OperandSize::Qword)
}

#[test]
fn pop_18() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1767630877, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 143, 4, 189, 29, 232, 91, 105], OperandSize::Qword)
}

#[test]
fn pop_19() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 89], OperandSize::Word)
}

#[test]
fn pop_20() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectDisplaced(SI, 24035, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 143, 132, 227, 93], OperandSize::Word)
}

#[test]
fn pop_21() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[89], OperandSize::Dword)
}

#[test]
fn pop_22() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[143, 4, 154], OperandSize::Dword)
}

#[test]
fn pop_23() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(RDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[90], OperandSize::Qword)
}

#[test]
fn pop_24() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 286804644, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[143, 4, 245, 164, 74, 24, 17], OperandSize::Qword)
}

#[test]
fn pop_25() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Word)
}

#[test]
fn pop_26() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Dword)
}

#[test]
fn pop_27() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Qword)
}

#[test]
fn pop_28() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Word)
}

#[test]
fn pop_29() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Dword)
}

#[test]
fn pop_30() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Qword)
}

#[test]
fn pop_31() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Word)
}

#[test]
fn pop_32() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Dword)
}

#[test]
fn pop_33() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Qword)
}

#[test]
fn pop_34() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Word)
}

#[test]
fn pop_35() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Dword)
}

#[test]
fn pop_36() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Qword)
}

