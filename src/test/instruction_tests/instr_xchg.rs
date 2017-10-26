use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 211], OperandSize::Word)
}

#[test]
fn xchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(BX, 239, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 151, 239, 0], OperandSize::Word)
}

#[test]
fn xchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 217], OperandSize::Dword)
}

#[test]
fn xchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 12, 67], OperandSize::Dword)
}

#[test]
fn xchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 219], OperandSize::Qword)
}

#[test]
fn xchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1201695839, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 12, 181, 95, 108, 160, 71], OperandSize::Qword)
}

#[test]
fn xchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 211], OperandSize::Qword)
}

#[test]
fn xchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 417794171, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 148, 115, 123, 8, 231, 24], OperandSize::Qword)
}

#[test]
fn xchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 201], OperandSize::Word)
}

#[test]
fn xchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(SI, 243, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 148, 243, 0], OperandSize::Word)
}

#[test]
fn xchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 209], OperandSize::Dword)
}

#[test]
fn xchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 26], OperandSize::Dword)
}

#[test]
fn xchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 209], OperandSize::Qword)
}

#[test]
fn xchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(RSI, 1427391937, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 158, 193, 69, 20, 85], OperandSize::Qword)
}

#[test]
fn xchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 219], OperandSize::Qword)
}

#[test]
fn xchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 3926112, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[134, 28, 133, 96, 232, 59, 0], OperandSize::Qword)
}

#[test]
fn xchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 211], OperandSize::Word)
}

#[test]
fn xchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 176, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 186, 176, 0], OperandSize::Word)
}

#[test]
fn xchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 215], OperandSize::Dword)
}

#[test]
fn xchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(EDX, 426017087, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 170, 63, 129, 100, 25], OperandSize::Dword)
}

#[test]
fn xchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 210], OperandSize::Qword)
}

#[test]
fn xchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1586593124, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 60, 93, 100, 125, 145, 94], OperandSize::Qword)
}

#[test]
fn xchg_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 206], OperandSize::Word)
}

#[test]
fn xchg_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(BP, 4291, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 166, 195, 16], OperandSize::Word)
}

#[test]
fn xchg_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 236], OperandSize::Dword)
}

#[test]
fn xchg_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 47], OperandSize::Dword)
}

#[test]
fn xchg_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 233], OperandSize::Qword)
}

#[test]
fn xchg_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 43], OperandSize::Qword)
}

#[test]
fn xchg_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 203], OperandSize::Word)
}

#[test]
fn xchg_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(BX, 28319, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 151, 159, 110], OperandSize::Word)
}

#[test]
fn xchg_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 246], OperandSize::Dword)
}

#[test]
fn xchg_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1998314677, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 140, 255, 181, 220, 27, 119], OperandSize::Dword)
}

#[test]
fn xchg_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 223], OperandSize::Qword)
}

#[test]
fn xchg_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 52, 134], OperandSize::Qword)
}

#[test]
fn xchg_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 219], OperandSize::Qword)
}

#[test]
fn xchg_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(IndirectDisplaced(RSI, 22353406, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 166, 254, 21, 85, 1], OperandSize::Qword)
}

#[test]
fn xchg_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 237], OperandSize::Word)
}

#[test]
fn xchg_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(BP, 27705, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 135, 150, 57, 108], OperandSize::Word)
}

#[test]
fn xchg_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 222], OperandSize::Dword)
}

#[test]
fn xchg_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 18], OperandSize::Dword)
}

#[test]
fn xchg_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 252], OperandSize::Qword)
}

#[test]
fn xchg_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[135, 60, 65], OperandSize::Qword)
}

#[test]
fn xchg_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 209], OperandSize::Qword)
}

#[test]
fn xchg_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1324780352, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 135, 44, 205, 64, 139, 246, 78], OperandSize::Qword)
}

#[test]
fn xchg_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[150], OperandSize::Word)
}

#[test]
fn xchg_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 150], OperandSize::Dword)
}

#[test]
fn xchg_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(AX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 145], OperandSize::Qword)
}

#[test]
fn xchg_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(SI)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[150], OperandSize::Word)
}

#[test]
fn xchg_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BP)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Dword)
}

#[test]
fn xchg_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(BX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 147], OperandSize::Qword)
}

#[test]
fn xchg_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 151], OperandSize::Word)
}

#[test]
fn xchg_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[148], OperandSize::Dword)
}

#[test]
fn xchg_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EAX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[150], OperandSize::Qword)
}

#[test]
fn xchg_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RAX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 148], OperandSize::Qword)
}

#[test]
fn xchg_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 149], OperandSize::Word)
}

#[test]
fn xchg_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[149], OperandSize::Dword)
}

#[test]
fn xchg_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(ESI)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[150], OperandSize::Qword)
}

#[test]
fn xchg_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XCHG, operand1: Some(Direct(RDX)), operand2: Some(Direct(RAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 146], OperandSize::Qword)
}

