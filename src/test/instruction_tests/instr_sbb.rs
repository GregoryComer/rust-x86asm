use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sbb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Word)
}

#[test]
fn sbb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 43, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 80, 43], OperandSize::Word)
}

#[test]
fn sbb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 209], OperandSize::Dword)
}

#[test]
fn sbb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(EBX, 978575838, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 155, 222, 225, 83, 58], OperandSize::Dword)
}

#[test]
fn sbb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Qword)
}

#[test]
fn sbb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 562751404, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 148, 138, 172, 231, 138, 33], OperandSize::Qword)
}

#[test]
fn sbb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 219], OperandSize::Qword)
}

#[test]
fn sbb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 19], OperandSize::Qword)
}

#[test]
fn sbb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 242], OperandSize::Word)
}

#[test]
fn sbb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 17545, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 187, 137, 68], OperandSize::Word)
}

#[test]
fn sbb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 214], OperandSize::Dword)
}

#[test]
fn sbb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 14], OperandSize::Dword)
}

#[test]
fn sbb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 213], OperandSize::Qword)
}

#[test]
fn sbb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RDI, 532938812, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 151, 60, 0, 196, 31], OperandSize::Qword)
}

#[test]
fn sbb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 241], OperandSize::Word)
}

#[test]
fn sbb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 2373, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 163, 69, 9], OperandSize::Word)
}

#[test]
fn sbb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 202], OperandSize::Dword)
}

#[test]
fn sbb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 12, 219], OperandSize::Dword)
}

#[test]
fn sbb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 204], OperandSize::Qword)
}

#[test]
fn sbb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 24], OperandSize::Qword)
}

#[test]
fn sbb_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 242], OperandSize::Qword)
}

#[test]
fn sbb_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 36, 214], OperandSize::Qword)
}

#[test]
fn sbb_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 209], OperandSize::Word)
}

#[test]
fn sbb_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(SI, 1255, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 156, 231, 4], OperandSize::Word)
}

#[test]
fn sbb_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Dword)
}

#[test]
fn sbb_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 12, 190], OperandSize::Dword)
}

#[test]
fn sbb_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Qword)
}

#[test]
fn sbb_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 20, 241], OperandSize::Qword)
}

#[test]
fn sbb_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Qword)
}

#[test]
fn sbb_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 17], OperandSize::Qword)
}

#[test]
fn sbb_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 238], OperandSize::Word)
}

#[test]
fn sbb_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(DI, 93, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 93, 93], OperandSize::Word)
}

#[test]
fn sbb_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 203], OperandSize::Dword)
}

#[test]
fn sbb_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 1992788540, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 148, 210, 60, 138, 199, 118], OperandSize::Dword)
}

#[test]
fn sbb_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 254], OperandSize::Qword)
}

#[test]
fn sbb_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 28, 81], OperandSize::Qword)
}

#[test]
fn sbb_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 204], OperandSize::Word)
}

#[test]
fn sbb_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 165, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 169, 165, 0], OperandSize::Word)
}

#[test]
fn sbb_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 235], OperandSize::Dword)
}

#[test]
fn sbb_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1893299186, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 28, 133, 242, 115, 217, 112], OperandSize::Dword)
}

#[test]
fn sbb_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 251], OperandSize::Qword)
}

#[test]
fn sbb_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1002234443, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 188, 74, 75, 226, 188, 59], OperandSize::Qword)
}

#[test]
fn sbb_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 231], OperandSize::Qword)
}

#[test]
fn sbb_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RBX, 1961059426, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 27, 139, 98, 100, 227, 116], OperandSize::Qword)
}

#[test]
fn sbb_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 103], OperandSize::Word)
}

#[test]
fn sbb_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 109], OperandSize::Dword)
}

#[test]
fn sbb_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 40], OperandSize::Qword)
}

#[test]
fn sbb_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(25083)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 251, 97], OperandSize::Word)
}

#[test]
fn sbb_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(11628)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 108, 45], OperandSize::Dword)
}

#[test]
fn sbb_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(14321)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 241, 55], OperandSize::Qword)
}

#[test]
fn sbb_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1269627997)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 93, 252, 172, 75], OperandSize::Word)
}

#[test]
fn sbb_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1579565378)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 66, 65, 38, 94], OperandSize::Dword)
}

#[test]
fn sbb_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1649513962)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 234, 149, 81, 98], OperandSize::Qword)
}

#[test]
fn sbb_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1323331837)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 29, 253, 112, 224, 78], OperandSize::Qword)
}

#[test]
fn sbb_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 219, 29], OperandSize::Word)
}

#[test]
fn sbb_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 13504, Some(OperandSize::Byte), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 152, 192, 52, 107], OperandSize::Word)
}

#[test]
fn sbb_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 15], OperandSize::Dword)
}

#[test]
fn sbb_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 27, 66], OperandSize::Dword)
}

#[test]
fn sbb_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 113], OperandSize::Qword)
}

#[test]
fn sbb_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1297031036, Some(OperandSize::Byte), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 189, 124, 31, 79, 77, 7], OperandSize::Qword)
}

#[test]
fn sbb_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 219, 48], OperandSize::Qword)
}

#[test]
fn sbb_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1530595755, Some(OperandSize::Byte), None)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 253, 171, 9, 59, 91, 88], OperandSize::Qword)
}

#[test]
fn sbb_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Literal16(3490)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 218, 162, 13], OperandSize::Word)
}

#[test]
fn sbb_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(SI, 10, Some(OperandSize::Word), None)), operand2: Some(Literal16(26586)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 92, 10, 218, 103], OperandSize::Word)
}

#[test]
fn sbb_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Literal16(9370)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 222, 154, 36], OperandSize::Dword)
}

#[test]
fn sbb_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1804741352, Some(OperandSize::Word), None)), operand2: Some(Literal16(9306)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 156, 89, 232, 42, 146, 107, 90, 36], OperandSize::Dword)
}

#[test]
fn sbb_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Literal16(20555)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 223, 75, 80], OperandSize::Qword)
}

#[test]
fn sbb_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(31094)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 28, 115, 118, 121], OperandSize::Qword)
}

#[test]
fn sbb_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1989610348)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 222, 108, 11, 151, 118], OperandSize::Word)
}

#[test]
fn sbb_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BP, 216, Some(OperandSize::Dword), None)), operand2: Some(Literal32(843918111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 158, 216, 0, 31, 43, 77, 50], OperandSize::Word)
}

#[test]
fn sbb_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1924225040)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 222, 16, 88, 177, 114], OperandSize::Dword)
}

#[test]
fn sbb_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1601311766, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1523137905)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 28, 69, 22, 20, 114, 95, 113, 61, 201, 90], OperandSize::Dword)
}

#[test]
fn sbb_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1837075470)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 222, 14, 140, 127, 109], OperandSize::Qword)
}

#[test]
fn sbb_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RBX, 2105135615, Some(OperandSize::Dword), None)), operand2: Some(Literal32(137151111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 155, 255, 209, 121, 125, 135, 194, 44, 8], OperandSize::Qword)
}

#[test]
fn sbb_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RDI)), operand2: Some(Literal32(436862870)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 223, 150, 255, 9, 26], OperandSize::Qword)
}

#[test]
fn sbb_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1000652532, Some(OperandSize::Qword), None)), operand2: Some(Literal32(562344148)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 28, 189, 244, 190, 164, 59, 212, 176, 132, 33], OperandSize::Qword)
}

#[test]
fn sbb_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 223, 47], OperandSize::Word)
}

#[test]
fn sbb_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 2, Some(OperandSize::Word), None)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 90, 2, 88], OperandSize::Word)
}

#[test]
fn sbb_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 223, 44], OperandSize::Dword)
}

#[test]
fn sbb_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1283061803, Some(OperandSize::Word), None)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 156, 255, 43, 248, 121, 76, 13], OperandSize::Dword)
}

#[test]
fn sbb_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 220, 58], OperandSize::Qword)
}

#[test]
fn sbb_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 28, 192, 113], OperandSize::Qword)
}

#[test]
fn sbb_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 223, 111], OperandSize::Word)
}

#[test]
fn sbb_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 254, Some(OperandSize::Dword), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 153, 254, 0, 47], OperandSize::Word)
}

#[test]
fn sbb_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 219, 20], OperandSize::Dword)
}

#[test]
fn sbb_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 28, 203, 27], OperandSize::Dword)
}

#[test]
fn sbb_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDX)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 218, 24], OperandSize::Qword)
}

#[test]
fn sbb_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 249777824, Some(OperandSize::Dword), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 156, 202, 160, 78, 227, 14, 83], OperandSize::Qword)
}

#[test]
fn sbb_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBX)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 219, 72], OperandSize::Qword)
}

#[test]
fn sbb_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 27, 49], OperandSize::Qword)
}

