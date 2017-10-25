use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 218], OperandSize::Word)
}

#[test]
fn xchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Memory(23371, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 22, 75, 91], OperandSize::Word)
}

#[test]
fn xchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 203], OperandSize::Dword)
}

#[test]
fn xchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 15], OperandSize::Dword)
}

#[test]
fn xchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 217], OperandSize::Qword)
}

#[test]
fn xchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(RDX, 1060349713, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 138, 17, 167, 51, 63], OperandSize::Qword)
}

#[test]
fn xchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 210], OperandSize::Qword)
}

#[test]
fn xchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(RSI, 1237396855, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 142, 119, 45, 193, 73], OperandSize::Qword)
}

#[test]
fn xchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 219], OperandSize::Word)
}

#[test]
fn xchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 170, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 152, 170, 0], OperandSize::Word)
}

#[test]
fn xchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 201], OperandSize::Dword)
}

#[test]
fn xchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 22], OperandSize::Dword)
}

#[test]
fn xchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 219], OperandSize::Qword)
}

#[test]
fn xchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 724574701, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 148, 216, 237, 33, 48, 43], OperandSize::Qword)
}

#[test]
fn xchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 209], OperandSize::Qword)
}

#[test]
fn xchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 16], OperandSize::Qword)
}

#[test]
fn xchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 206], OperandSize::Word)
}

#[test]
fn xchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(DI, 11185, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 149, 177, 43], OperandSize::Word)
}

#[test]
fn xchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 213], OperandSize::Dword)
}

#[test]
fn xchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(EDX, Two, 2113130720, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 52, 85, 224, 208, 243, 125], OperandSize::Dword)
}

#[test]
fn xchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 225], OperandSize::Qword)
}

#[test]
fn xchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(RBX, Four, 59600149, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 20, 157, 21, 109, 141, 3], OperandSize::Qword)
}

#[test]
fn xchg_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 235], OperandSize::Word)
}

#[test]
fn xchg_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 782, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 154, 14, 3], OperandSize::Word)
}

#[test]
fn xchg_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 249], OperandSize::Dword)
}

#[test]
fn xchg_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 767863596, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 28, 213, 44, 171, 196, 45], OperandSize::Dword)
}

#[test]
fn xchg_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 201], OperandSize::Qword)
}

#[test]
fn xchg_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 52, 198], OperandSize::Qword)
}

#[test]
fn xchg_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 237], OperandSize::Word)
}

#[test]
fn xchg_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 57], OperandSize::Word)
}

#[test]
fn xchg_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 239], OperandSize::Dword)
}

#[test]
fn xchg_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 23], OperandSize::Dword)
}

#[test]
fn xchg_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 250], OperandSize::Qword)
}

#[test]
fn xchg_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 367894161, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 28, 221, 145, 158, 237, 21], OperandSize::Qword)
}

#[test]
fn xchg_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 239], OperandSize::Qword)
}

#[test]
fn xchg_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 50], OperandSize::Qword)
}

#[test]
fn xchg_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 247], OperandSize::Word)
}

#[test]
fn xchg_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 193, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 168, 193, 0], OperandSize::Word)
}

#[test]
fn xchg_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 211], OperandSize::Dword)
}

#[test]
fn xchg_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1942228000, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 148, 151, 32, 12, 196, 115], OperandSize::Dword)
}

#[test]
fn xchg_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 241], OperandSize::Qword)
}

#[test]
fn xchg_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1481956719, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 28, 117, 111, 221, 84, 88], OperandSize::Qword)
}

#[test]
fn xchg_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 210], OperandSize::Qword)
}

#[test]
fn xchg_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 590928646, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 172, 190, 6, 219, 56, 35], OperandSize::Qword)
}

#[test]
fn xchg_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[148], OperandSize::Word)
}

#[test]
fn xchg_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Dword)
}

#[test]
fn xchg_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 147], OperandSize::Qword)
}

#[test]
fn xchg_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[148], OperandSize::Word)
}

#[test]
fn xchg_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Dword)
}

#[test]
fn xchg_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Qword)
}

#[test]
fn xchg_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Word)
}

#[test]
fn xchg_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[149], OperandSize::Dword)
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
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 151], OperandSize::Word)
}

#[test]
fn xchg_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[150], OperandSize::Dword)
}

#[test]
fn xchg_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[149], OperandSize::Qword)
}

#[test]
fn xchg_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RSI)), operand2: Some(Direct(RAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 150], OperandSize::Qword)
}

