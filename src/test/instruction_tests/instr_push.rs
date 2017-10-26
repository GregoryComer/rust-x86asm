use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn push_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(ES)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[6], OperandSize::Word)
}

#[test]
fn push_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(ES)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[6], OperandSize::Dword)
}

#[test]
fn push_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(CS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[14], OperandSize::Word)
}

#[test]
fn push_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(CS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[14], OperandSize::Dword)
}

#[test]
fn push_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(SS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[22], OperandSize::Word)
}

#[test]
fn push_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(SS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[22], OperandSize::Dword)
}

#[test]
fn push_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(DS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[30], OperandSize::Word)
}

#[test]
fn push_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(DS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[30], OperandSize::Dword)
}

#[test]
fn push_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[81], OperandSize::Word)
}

#[test]
fn push_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 81], OperandSize::Dword)
}

#[test]
fn push_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 85], OperandSize::Qword)
}

#[test]
fn push_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 84], OperandSize::Word)
}

#[test]
fn push_13() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[86], OperandSize::Dword)
}

#[test]
fn push_14() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[84], OperandSize::Qword)
}

#[test]
fn push_15() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal16(23859)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[104, 51, 93], OperandSize::Word)
}

#[test]
fn push_16() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal16(15936)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[104, 64, 62, 0, 0], OperandSize::Dword)
}

#[test]
fn push_17() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal16(3941)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[104, 101, 15, 0, 0], OperandSize::Qword)
}

#[test]
fn push_18() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal32(443165520)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[104, 80, 43], OperandSize::Word)
}

#[test]
fn push_19() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal32(1544997829)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[104, 197, 203, 22, 92], OperandSize::Dword)
}

#[test]
fn push_20() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal32(878733564)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[104, 252, 104, 96, 52], OperandSize::Qword)
}

#[test]
fn push_21() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal8(29)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[106, 29], OperandSize::Word)
}

#[test]
fn push_22() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal8(127)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[106, 127], OperandSize::Dword)
}

#[test]
fn push_23() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Literal8(53)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[106, 53], OperandSize::Qword)
}

#[test]
fn push_24() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 160], OperandSize::Word)
}

#[test]
fn push_25() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 160], OperandSize::Dword)
}

#[test]
fn push_26() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 160], OperandSize::Qword)
}

#[test]
fn push_27() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 168], OperandSize::Word)
}

#[test]
fn push_28() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 168], OperandSize::Dword)
}

#[test]
fn push_29() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 168], OperandSize::Qword)
}

#[test]
fn push_30() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[84], OperandSize::Word)
}

#[test]
fn push_31() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 49], OperandSize::Word)
}

#[test]
fn push_32() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 85], OperandSize::Dword)
}

#[test]
fn push_33() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(IndirectDisplaced(ECX, 1157704592, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 177, 144, 43, 1, 69], OperandSize::Dword)
}

#[test]
fn push_34() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 87], OperandSize::Qword)
}

#[test]
fn push_35() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 49], OperandSize::Qword)
}

#[test]
fn push_36() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 82], OperandSize::Word)
}

#[test]
fn push_37() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 53], OperandSize::Word)
}

#[test]
fn push_38() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[87], OperandSize::Dword)
}

#[test]
fn push_39() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 52, 216], OperandSize::Dword)
}

#[test]
fn push_40() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(Direct(RDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[82], OperandSize::Qword)
}

#[test]
fn push_41() {
    run_test(&Instruction { mnemonic: Mnemonic::PUSH, operand1: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 52, 89], OperandSize::Qword)
}

