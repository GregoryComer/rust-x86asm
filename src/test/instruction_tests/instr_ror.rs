use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ror_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 202, 26], OperandSize::Word)
}

#[test]
fn ror_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BX, 178, Some(OperandSize::Byte), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 143, 178, 0, 117], OperandSize::Word)
}

#[test]
fn ror_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 201, 7], OperandSize::Dword)
}

#[test]
fn ror_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(ESI, Two, 10562306, Some(OperandSize::Byte), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 117, 2, 43, 161, 0, 23], OperandSize::Dword)
}

#[test]
fn ror_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 202, 52], OperandSize::Qword)
}

#[test]
fn ror_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 759802262, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 140, 81, 150, 169, 73, 45, 50], OperandSize::Qword)
}

#[test]
fn ror_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 201, 19], OperandSize::Qword)
}

#[test]
fn ror_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 12, 185, 33], OperandSize::Qword)
}

#[test]
fn ror_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CX)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 201, 49], OperandSize::Word)
}

#[test]
fn ror_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 10, 12], OperandSize::Word)
}

#[test]
fn ror_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DI)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 207, 92], OperandSize::Dword)
}

#[test]
fn ror_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 12, 151, 120], OperandSize::Dword)
}

#[test]
fn ror_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 206, 3], OperandSize::Qword)
}

#[test]
fn ror_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 9943625, Some(OperandSize::Word), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 140, 64, 73, 186, 151, 0, 35], OperandSize::Qword)
}

#[test]
fn ror_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 202, 100], OperandSize::Word)
}

#[test]
fn ror_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Memory(10063, Some(OperandSize::Dword), None)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 14, 79, 39, 91], OperandSize::Word)
}

#[test]
fn ror_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 207, 114], OperandSize::Dword)
}

#[test]
fn ror_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 74926389, Some(OperandSize::Dword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 140, 90, 53, 73, 119, 4, 92], OperandSize::Dword)
}

#[test]
fn ror_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 205, 114], OperandSize::Qword)
}

#[test]
fn ror_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 2015011067, Some(OperandSize::Dword), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 140, 71, 251, 160, 26, 120, 20], OperandSize::Qword)
}

#[test]
fn ror_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 203, 10], OperandSize::Qword)
}

#[test]
fn ror_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 12, 208, 71], OperandSize::Qword)
}

#[test]
fn ror_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 203], OperandSize::Word)
}

#[test]
fn ror_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 25773, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 136, 173, 100], OperandSize::Word)
}

#[test]
fn ror_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 203], OperandSize::Dword)
}

#[test]
fn ror_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(EAX, 290294736, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 136, 208, 139, 77, 17], OperandSize::Dword)
}

#[test]
fn ror_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 201], OperandSize::Qword)
}

#[test]
fn ror_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 11], OperandSize::Qword)
}

#[test]
fn ror_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 202], OperandSize::Qword)
}

#[test]
fn ror_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 226989666, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 12, 125, 98, 150, 135, 13], OperandSize::Qword)
}

#[test]
fn ror_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 202], OperandSize::Word)
}

#[test]
fn ror_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(DI, 222, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 141, 222, 0], OperandSize::Word)
}

#[test]
fn ror_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 202], OperandSize::Dword)
}

#[test]
fn ror_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 12, 211], OperandSize::Dword)
}

#[test]
fn ror_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 206], OperandSize::Qword)
}

#[test]
fn ror_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1101598809, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 12, 149, 89, 16, 169, 65], OperandSize::Qword)
}

#[test]
fn ror_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 201], OperandSize::Word)
}

#[test]
fn ror_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 15], OperandSize::Word)
}

#[test]
fn ror_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 202], OperandSize::Dword)
}

#[test]
fn ror_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1496581912, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 140, 177, 24, 7, 52, 89], OperandSize::Dword)
}

#[test]
fn ror_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 201], OperandSize::Qword)
}

#[test]
fn ror_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1532234850, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 12, 157, 98, 12, 84, 91], OperandSize::Qword)
}

#[test]
fn ror_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 207], OperandSize::Qword)
}

#[test]
fn ror_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 186264302, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 12, 77, 238, 42, 26, 11], OperandSize::Qword)
}

#[test]
fn ror_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 203], OperandSize::Word)
}

#[test]
fn ror_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(DI, 22274, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 141, 2, 87], OperandSize::Word)
}

#[test]
fn ror_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Dword)
}

#[test]
fn ror_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 12, 186], OperandSize::Dword)
}

#[test]
fn ror_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 201], OperandSize::Qword)
}

#[test]
fn ror_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1781909660, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 12, 93, 156, 200, 53, 106], OperandSize::Qword)
}

#[test]
fn ror_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 202], OperandSize::Qword)
}

#[test]
fn ror_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 12, 198], OperandSize::Qword)
}

#[test]
fn ror_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 205], OperandSize::Word)
}

#[test]
fn ror_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(DI, 7189, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 141, 21, 28], OperandSize::Word)
}

#[test]
fn ror_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 203], OperandSize::Dword)
}

#[test]
fn ror_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1236721819, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 12, 245, 155, 224, 182, 73], OperandSize::Dword)
}

#[test]
fn ror_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 203], OperandSize::Qword)
}

#[test]
fn ror_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 935549910, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 140, 114, 214, 91, 195, 55], OperandSize::Qword)
}

#[test]
fn ror_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 202], OperandSize::Word)
}

#[test]
fn ror_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(BP, 214, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 142, 214, 0], OperandSize::Word)
}

#[test]
fn ror_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 206], OperandSize::Dword)
}

#[test]
fn ror_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 2020610727, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 140, 74, 167, 18, 112, 120], OperandSize::Dword)
}

#[test]
fn ror_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 207], OperandSize::Qword)
}

#[test]
fn ror_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(IndirectDisplaced(RBX, 2082083041, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 139, 225, 16, 26, 124], OperandSize::Qword)
}

#[test]
fn ror_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Direct(RDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 202], OperandSize::Qword)
}

#[test]
fn ror_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROR, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 9], OperandSize::Qword)
}

