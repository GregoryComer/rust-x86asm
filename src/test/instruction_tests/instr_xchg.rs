use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 203], OperandSize::Word)
}

#[test]
fn xchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(BP, 173, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 158, 173, 0], OperandSize::Word)
}

#[test]
fn xchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 202], OperandSize::Dword)
}

#[test]
fn xchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1684794481, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 156, 208, 113, 236, 107, 100], OperandSize::Dword)
}

#[test]
fn xchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 209], OperandSize::Qword)
}

#[test]
fn xchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 11], OperandSize::Qword)
}

#[test]
fn xchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 201], OperandSize::Qword)
}

#[test]
fn xchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 23], OperandSize::Qword)
}

#[test]
fn xchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 209], OperandSize::Word)
}

#[test]
fn xchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 338, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 152, 82, 1], OperandSize::Word)
}

#[test]
fn xchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 218], OperandSize::Dword)
}

#[test]
fn xchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 19], OperandSize::Dword)
}

#[test]
fn xchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 210], OperandSize::Qword)
}

#[test]
fn xchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 28, 193], OperandSize::Qword)
}

#[test]
fn xchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 210], OperandSize::Qword)
}

#[test]
fn xchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 24], OperandSize::Qword)
}

#[test]
fn xchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 234], OperandSize::Word)
}

#[test]
fn xchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 191, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 144, 191, 0], OperandSize::Word)
}

#[test]
fn xchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 220], OperandSize::Dword)
}

#[test]
fn xchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(EDX, Four, 170822209, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 52, 149, 65, 138, 46, 10], OperandSize::Dword)
}

#[test]
fn xchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 252], OperandSize::Qword)
}

#[test]
fn xchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1749670492, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 60, 205, 92, 218, 73, 104], OperandSize::Qword)
}

#[test]
fn xchg_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 204], OperandSize::Word)
}

#[test]
fn xchg_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BP, 111, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 110, 111], OperandSize::Word)
}

#[test]
fn xchg_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 226], OperandSize::Dword)
}

#[test]
fn xchg_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1065514827, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 188, 187, 75, 119, 130, 63], OperandSize::Dword)
}

#[test]
fn xchg_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 207], OperandSize::Qword)
}

#[test]
fn xchg_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RDX, 2007457379, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 138, 99, 94, 167, 119], OperandSize::Qword)
}

#[test]
fn xchg_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 219], OperandSize::Word)
}

#[test]
fn xchg_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 22608, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 147, 80, 88], OperandSize::Word)
}

#[test]
fn xchg_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 233], OperandSize::Dword)
}

#[test]
fn xchg_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1799396914, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 180, 159, 50, 158, 64, 107], OperandSize::Dword)
}

#[test]
fn xchg_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 201], OperandSize::Qword)
}

#[test]
fn xchg_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(RAX, 1240115230, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 184, 30, 168, 234, 73], OperandSize::Qword)
}

#[test]
fn xchg_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 235], OperandSize::Qword)
}

#[test]
fn xchg_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 33], OperandSize::Qword)
}

#[test]
fn xchg_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 252], OperandSize::Word)
}

#[test]
fn xchg_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 43], OperandSize::Word)
}

#[test]
fn xchg_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 247], OperandSize::Dword)
}

#[test]
fn xchg_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESP)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 33], OperandSize::Dword)
}

#[test]
fn xchg_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 250], OperandSize::Qword)
}

#[test]
fn xchg_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1567123058, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 156, 250, 114, 102, 104, 93], OperandSize::Qword)
}

#[test]
fn xchg_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 212], OperandSize::Qword)
}

#[test]
fn xchg_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 996405145, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 12, 117, 153, 239, 99, 59], OperandSize::Qword)
}

#[test]
fn xchg_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[151], OperandSize::Word)
}

#[test]
fn xchg_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 150], OperandSize::Dword)
}

#[test]
fn xchg_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 150], OperandSize::Qword)
}

#[test]
fn xchg_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[148], OperandSize::Word)
}

#[test]
fn xchg_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 145], OperandSize::Dword)
}

#[test]
fn xchg_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 148], OperandSize::Qword)
}

#[test]
fn xchg_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Word)
}

#[test]
fn xchg_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[146], OperandSize::Dword)
}

#[test]
fn xchg_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[145], OperandSize::Qword)
}

#[test]
fn xchg_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RAX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 150], OperandSize::Qword)
}

#[test]
fn xchg_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 146], OperandSize::Word)
}

#[test]
fn xchg_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[147], OperandSize::Dword)
}

#[test]
fn xchg_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[147], OperandSize::Qword)
}

#[test]
fn xchg_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RDX)), operand2: Some(Direct(RAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 146], OperandSize::Qword)
}

