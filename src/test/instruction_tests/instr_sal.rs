use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sal_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 46], OperandSize::Word)
}

#[test]
fn sal_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 12, Some(OperandSize::Byte), None)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 98, 12, 72], OperandSize::Word)
}

#[test]
fn sal_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 13], OperandSize::Dword)
}

#[test]
fn sal_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(EDI, Four, 728877408, Some(OperandSize::Byte), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 189, 96, 201, 113, 43, 126], OperandSize::Dword)
}

#[test]
fn sal_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 94], OperandSize::Qword)
}

#[test]
fn sal_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 33, 107], OperandSize::Qword)
}

#[test]
fn sal_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 21], OperandSize::Qword)
}

#[test]
fn sal_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 572518960, Some(OperandSize::Byte), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 164, 128, 48, 242, 31, 34, 83], OperandSize::Qword)
}

#[test]
fn sal_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(SP)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 228, 78], OperandSize::Word)
}

#[test]
fn sal_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 242, Some(OperandSize::Word), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 163, 242, 0, 45], OperandSize::Word)
}

#[test]
fn sal_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DI)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 231, 72], OperandSize::Dword)
}

#[test]
fn sal_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(ESI, 1973763249, Some(OperandSize::Word), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 166, 177, 60, 165, 117, 75], OperandSize::Dword)
}

#[test]
fn sal_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DI)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 231, 62], OperandSize::Qword)
}

#[test]
fn sal_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1253380208, Some(OperandSize::Word), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 164, 74, 112, 16, 181, 74, 115], OperandSize::Qword)
}

#[test]
fn sal_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 227, 85], OperandSize::Word)
}

#[test]
fn sal_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 32, 18], OperandSize::Word)
}

#[test]
fn sal_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 9], OperandSize::Dword)
}

#[test]
fn sal_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(ECX, Two, 429888043, Some(OperandSize::Dword), None)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 36, 77, 43, 146, 159, 25, 127], OperandSize::Dword)
}

#[test]
fn sal_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 228, 46], OperandSize::Qword)
}

#[test]
fn sal_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 18556747, Some(OperandSize::Dword), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 36, 117, 75, 39, 27, 1, 14], OperandSize::Qword)
}

#[test]
fn sal_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RSP)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 228, 0], OperandSize::Qword)
}

#[test]
fn sal_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 32, 34], OperandSize::Qword)
}

#[test]
fn sal_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Word)
}

#[test]
fn sal_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BP, 5442, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 166, 66, 21], OperandSize::Word)
}

#[test]
fn sal_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Dword)
}

#[test]
fn sal_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 33], OperandSize::Dword)
}

#[test]
fn sal_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn sal_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 113], OperandSize::Qword)
}

#[test]
fn sal_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn sal_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 154], OperandSize::Qword)
}

#[test]
fn sal_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 231], OperandSize::Word)
}

#[test]
fn sal_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 32], OperandSize::Word)
}

#[test]
fn sal_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 229], OperandSize::Dword)
}

#[test]
fn sal_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(EDI, 1143699362, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 167, 162, 119, 43, 68], OperandSize::Dword)
}

#[test]
fn sal_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Qword)
}

#[test]
fn sal_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1406351648, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 164, 118, 32, 57, 211, 83], OperandSize::Qword)
}

#[test]
fn sal_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 230], OperandSize::Word)
}

#[test]
fn sal_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(SI, 68, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 100, 68], OperandSize::Word)
}

#[test]
fn sal_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 227], OperandSize::Dword)
}

#[test]
fn sal_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(EAX, Two, 158486147, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 69, 131, 78, 114, 9], OperandSize::Dword)
}

#[test]
fn sal_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Qword)
}

#[test]
fn sal_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 811604364, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 205, 140, 25, 96, 48], OperandSize::Qword)
}

#[test]
fn sal_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RSP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 228], OperandSize::Qword)
}

#[test]
fn sal_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1272762173, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 164, 193, 61, 207, 220, 75], OperandSize::Qword)
}

#[test]
fn sal_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Word)
}

#[test]
fn sal_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 37], OperandSize::Word)
}

#[test]
fn sal_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Dword)
}

#[test]
fn sal_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(EBX, 1051688209, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 163, 17, 125, 175, 62], OperandSize::Dword)
}

#[test]
fn sal_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn sal_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RCX, 1491457323, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 161, 43, 213, 229, 88], OperandSize::Qword)
}

#[test]
fn sal_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn sal_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1740712580, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 178, 132, 42, 193, 103], OperandSize::Qword)
}

#[test]
fn sal_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 225], OperandSize::Word)
}

#[test]
fn sal_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BP, 17864, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 166, 200, 69], OperandSize::Word)
}

#[test]
fn sal_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 229], OperandSize::Dword)
}

#[test]
fn sal_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 38], OperandSize::Dword)
}

#[test]
fn sal_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 229], OperandSize::Qword)
}

#[test]
fn sal_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 730062682, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 36, 117, 90, 223, 131, 43], OperandSize::Qword)
}

#[test]
fn sal_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 229], OperandSize::Word)
}

#[test]
fn sal_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 36], OperandSize::Word)
}

#[test]
fn sal_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 231], OperandSize::Dword)
}

#[test]
fn sal_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 2050470039, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 164, 130, 151, 176, 55, 122], OperandSize::Dword)
}

#[test]
fn sal_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 225], OperandSize::Qword)
}

#[test]
fn sal_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 454079714, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36, 197, 226, 180, 16, 27], OperandSize::Qword)
}

#[test]
fn sal_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 231], OperandSize::Qword)
}

#[test]
fn sal_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1829307959, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 36, 133, 55, 6, 9, 109], OperandSize::Qword)
}

