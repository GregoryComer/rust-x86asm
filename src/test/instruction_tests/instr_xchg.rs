use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 217], OperandSize::Word)
}

#[test]
fn xchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 31], OperandSize::Word)
}

#[test]
fn xchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 211], OperandSize::Dword)
}

#[test]
fn xchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(ESI, 607557252, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 150, 132, 150, 54, 36], OperandSize::Dword)
}

#[test]
fn xchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 219], OperandSize::Qword)
}

#[test]
fn xchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 27], OperandSize::Qword)
}

#[test]
fn xchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 219], OperandSize::Qword)
}

#[test]
fn xchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 49097407, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 156, 190, 191, 42, 237, 2], OperandSize::Qword)
}

#[test]
fn xchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 202], OperandSize::Word)
}

#[test]
fn xchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Memory(26385, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 30, 17, 103], OperandSize::Word)
}

#[test]
fn xchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 202], OperandSize::Dword)
}

#[test]
fn xchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(EBX, 1771909240, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 139, 120, 48, 157, 105], OperandSize::Dword)
}

#[test]
fn xchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 209], OperandSize::Qword)
}

#[test]
fn xchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RSI, 836621875, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 142, 51, 214, 221, 49], OperandSize::Qword)
}

#[test]
fn xchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 210], OperandSize::Qword)
}

#[test]
fn xchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 28, 247], OperandSize::Qword)
}

#[test]
fn xchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 251], OperandSize::Word)
}

#[test]
fn xchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 24113, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 185, 49, 94], OperandSize::Word)
}

#[test]
fn xchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 219], OperandSize::Dword)
}

#[test]
fn xchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 58], OperandSize::Dword)
}

#[test]
fn xchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 242], OperandSize::Qword)
}

#[test]
fn xchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1122272938, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 172, 200, 170, 134, 228, 66], OperandSize::Qword)
}

#[test]
fn xchg_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 235], OperandSize::Word)
}

#[test]
fn xchg_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 11], OperandSize::Word)
}

#[test]
fn xchg_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 211], OperandSize::Dword)
}

#[test]
fn xchg_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1949251873, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 164, 131, 33, 57, 47, 116], OperandSize::Dword)
}

#[test]
fn xchg_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 204], OperandSize::Qword)
}

#[test]
fn xchg_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1026160984, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 164, 155, 88, 249, 41, 61], OperandSize::Qword)
}

#[test]
fn xchg_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 247], OperandSize::Word)
}

#[test]
fn xchg_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 56], OperandSize::Word)
}

#[test]
fn xchg_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 213], OperandSize::Dword)
}

#[test]
fn xchg_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 276782274, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 36, 221, 194, 92, 127, 16], OperandSize::Dword)
}

#[test]
fn xchg_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 214], OperandSize::Qword)
}

#[test]
fn xchg_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1718194188, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 172, 90, 12, 144, 105, 102], OperandSize::Qword)
}

#[test]
fn xchg_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 219], OperandSize::Qword)
}

#[test]
fn xchg_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 60, 72], OperandSize::Qword)
}

#[test]
fn xchg_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 212], OperandSize::Word)
}

#[test]
fn xchg_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(BP, 21473, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 166, 225, 83], OperandSize::Word)
}

#[test]
fn xchg_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 205], OperandSize::Dword)
}

#[test]
fn xchg_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 1848360702, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 153, 254, 190, 43, 110], OperandSize::Dword)
}

#[test]
fn xchg_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 206], OperandSize::Qword)
}

#[test]
fn xchg_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 12, 243], OperandSize::Qword)
}

#[test]
fn xchg_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 201], OperandSize::Qword)
}

#[test]
fn xchg_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDX, 1097882242, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 138, 130, 90, 112, 65], OperandSize::Qword)
}

#[test]
fn xchg_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[150], OperandSize::Word)
}

#[test]
fn xchg_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Dword)
}

#[test]
fn xchg_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Qword)
}

#[test]
fn xchg_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[146], OperandSize::Word)
}

#[test]
fn xchg_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 145], OperandSize::Dword)
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
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[147], OperandSize::Dword)
}

#[test]
fn xchg_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[151], OperandSize::Qword)
}

#[test]
fn xchg_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RAX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 148], OperandSize::Qword)
}

#[test]
fn xchg_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 146], OperandSize::Word)
}

#[test]
fn xchg_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ECX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[145], OperandSize::Dword)
}

#[test]
fn xchg_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[147], OperandSize::Qword)
}

#[test]
fn xchg_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RSI)), operand2: Some(Direct(RAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 150], OperandSize::Qword)
}

