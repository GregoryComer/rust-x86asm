use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ror_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 70], OperandSize::Word)
}

#[test]
fn ror_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 11, 89], OperandSize::Word)
}

#[test]
fn ror_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 5], OperandSize::Dword)
}

#[test]
fn ror_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 155, 117], OperandSize::Dword)
}

#[test]
fn ror_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 203, 44], OperandSize::Qword)
}

#[test]
fn ror_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 506731027, Some(OperandSize::Byte), None)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 125, 19, 26, 52, 30, 95], OperandSize::Qword)
}

#[test]
fn ror_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Qword)
}

#[test]
fn ror_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 83, 21], OperandSize::Qword)
}

#[test]
fn ror_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BP)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 205, 21], OperandSize::Word)
}

#[test]
fn ror_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Memory(14979, Some(OperandSize::Word), None)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 14, 131, 58, 59], OperandSize::Word)
}

#[test]
fn ror_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 202, 123], OperandSize::Dword)
}

#[test]
fn ror_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 460737345, Some(OperandSize::Word), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 140, 184, 65, 75, 118, 27, 2], OperandSize::Dword)
}

#[test]
fn ror_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CX)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 201, 77], OperandSize::Qword)
}

#[test]
fn ror_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RBX, 1994932295, Some(OperandSize::Word), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 139, 71, 64, 232, 118, 20], OperandSize::Qword)
}

#[test]
fn ror_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 205, 22], OperandSize::Word)
}

#[test]
fn ror_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 10, 37], OperandSize::Word)
}

#[test]
fn ror_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 201, 100], OperandSize::Dword)
}

#[test]
fn ror_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 12, 206, 93], OperandSize::Dword)
}

#[test]
fn ror_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 202, 52], OperandSize::Qword)
}

#[test]
fn ror_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 8, 19], OperandSize::Qword)
}

#[test]
fn ror_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RSP)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 204, 90], OperandSize::Qword)
}

#[test]
fn ror_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 607952839, Some(OperandSize::Qword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 12, 77, 199, 159, 60, 36, 92], OperandSize::Qword)
}

#[test]
fn ror_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Word)
}

#[test]
fn ror_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 13], OperandSize::Word)
}

#[test]
fn ror_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Dword)
}

#[test]
fn ror_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(EDI, 170932277, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 143, 53, 56, 48, 10], OperandSize::Dword)
}

#[test]
fn ror_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 203], OperandSize::Qword)
}

#[test]
fn ror_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1854364338, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 140, 144, 178, 90, 135, 110], OperandSize::Qword)
}

#[test]
fn ror_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Qword)
}

#[test]
fn ror_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 549252047, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 12, 133, 207, 235, 188, 32], OperandSize::Qword)
}

#[test]
fn ror_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 201], OperandSize::Word)
}

#[test]
fn ror_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 9], OperandSize::Word)
}

#[test]
fn ror_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 207], OperandSize::Dword)
}

#[test]
fn ror_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 8], OperandSize::Dword)
}

#[test]
fn ror_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 206], OperandSize::Qword)
}

#[test]
fn ror_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RBX, 642322368, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 139, 192, 15, 73, 38], OperandSize::Qword)
}

#[test]
fn ror_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 202], OperandSize::Word)
}

#[test]
fn ror_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 2590, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 136, 30, 10], OperandSize::Word)
}

#[test]
fn ror_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 205], OperandSize::Dword)
}

#[test]
fn ror_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 732723971, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 140, 131, 3, 123, 172, 43], OperandSize::Dword)
}

#[test]
fn ror_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 204], OperandSize::Qword)
}

#[test]
fn ror_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RAX, 1608908137, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 136, 105, 253, 229, 95], OperandSize::Qword)
}

#[test]
fn ror_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 207], OperandSize::Qword)
}

#[test]
fn ror_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 10], OperandSize::Qword)
}

#[test]
fn ror_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Word)
}

#[test]
fn ror_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 11], OperandSize::Word)
}

#[test]
fn ror_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 203], OperandSize::Dword)
}

#[test]
fn ror_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(EDI, 334936806, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 143, 230, 186, 246, 19], OperandSize::Dword)
}

#[test]
fn ror_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 202], OperandSize::Qword)
}

#[test]
fn ror_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 12, 155], OperandSize::Qword)
}

#[test]
fn ror_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Qword)
}

#[test]
fn ror_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RDI, 576754026, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 143, 106, 145, 96, 34], OperandSize::Qword)
}

#[test]
fn ror_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 205], OperandSize::Word)
}

#[test]
fn ror_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12228, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 139, 196, 47], OperandSize::Word)
}

#[test]
fn ror_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 201], OperandSize::Dword)
}

#[test]
fn ror_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 493258198, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 12, 125, 214, 133, 102, 29], OperandSize::Dword)
}

#[test]
fn ror_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 201], OperandSize::Qword)
}

#[test]
fn ror_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 25505037, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 12, 125, 13, 45, 133, 1], OperandSize::Qword)
}

#[test]
fn ror_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 201], OperandSize::Word)
}

#[test]
fn ror_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BP, 22227, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 142, 211, 86], OperandSize::Word)
}

#[test]
fn ror_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 207], OperandSize::Dword)
}

#[test]
fn ror_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(EDX, 1205827240, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 138, 168, 118, 223, 71], OperandSize::Dword)
}

#[test]
fn ror_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 205], OperandSize::Qword)
}

#[test]
fn ror_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1855719602, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 12, 93, 178, 8, 156, 110], OperandSize::Qword)
}

#[test]
fn ror_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RSP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 204], OperandSize::Qword)
}

#[test]
fn ror_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RDI, 287887099, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 143, 251, 206, 40, 17], OperandSize::Qword)
}

