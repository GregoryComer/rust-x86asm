use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn add_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Word)
}

fn add_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(BX, 12673, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 151, 129, 49], OperandSize::Word)
}

fn add_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Dword)
}

fn add_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1316378861, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 28, 149, 237, 88, 118, 78], OperandSize::Dword)
}

fn add_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 211], OperandSize::Qword)
}

fn add_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 12, 194], OperandSize::Qword)
}

fn add_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Qword)
}

fn add_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RDX, Four, 824760479, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 20, 149, 159, 216, 40, 49], OperandSize::Qword)
}

fn add_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 207], OperandSize::Word)
}

fn add_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 30438, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 188, 230, 118], OperandSize::Word)
}

fn add_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 213], OperandSize::Dword)
}

fn add_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 195997685, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 148, 86, 245, 175, 174, 11], OperandSize::Dword)
}

fn add_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 217], OperandSize::Qword)
}

fn add_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 476065764, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 44, 245, 228, 47, 96, 28], OperandSize::Qword)
}

fn add_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 236], OperandSize::Word)
}

fn add_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(BX, 93, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 111, 93], OperandSize::Word)
}

fn add_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 222], OperandSize::Dword)
}

fn add_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(ECX, 27565623, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 185, 55, 158, 164, 1], OperandSize::Dword)
}

fn add_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 245], OperandSize::Qword)
}

fn add_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1515813840, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 12, 125, 208, 123, 89, 90], OperandSize::Qword)
}

fn add_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 234], OperandSize::Qword)
}

fn add_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 792446267, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 172, 249, 59, 197, 59, 47], OperandSize::Qword)
}

fn add_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Word)
}

fn add_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 19], OperandSize::Word)
}

fn add_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Dword)
}

fn add_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1279085032, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 12, 141, 232, 73, 61, 76], OperandSize::Dword)
}

fn add_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Qword)
}

fn add_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 12, 151], OperandSize::Qword)
}

fn add_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Qword)
}

fn add_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1137463649, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 20, 157, 97, 81, 204, 67], OperandSize::Qword)
}

fn add_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 238], OperandSize::Word)
}

fn add_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(DI, 1743, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 173, 207, 6], OperandSize::Word)
}

fn add_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 218], OperandSize::Dword)
}

fn add_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(EDI, 346507599, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 175, 79, 73, 167, 20], OperandSize::Dword)
}

fn add_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 230], OperandSize::Qword)
}

fn add_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 48], OperandSize::Qword)
}

fn add_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 211], OperandSize::Word)
}

fn add_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BP, 26252, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 158, 140, 102], OperandSize::Word)
}

fn add_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 215], OperandSize::Dword)
}

fn add_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 587408185, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 156, 243, 57, 35, 3, 35], OperandSize::Dword)
}

fn add_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 242], OperandSize::Qword)
}

fn add_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1659979746, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 20, 197, 226, 71, 241, 98], OperandSize::Qword)
}

fn add_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 255], OperandSize::Qword)
}

fn add_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 3, 9], OperandSize::Qword)
}

fn add_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 8], OperandSize::Word)
}

fn add_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 76], OperandSize::Dword)
}

fn add_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 28], OperandSize::Qword)
}

fn add_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(18044)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 124, 70], OperandSize::Word)
}

fn add_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(26641)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 17, 104], OperandSize::Dword)
}

fn add_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(11723)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 203, 45], OperandSize::Qword)
}

fn add_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(275662661)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 69, 71, 110, 16], OperandSize::Word)
}

fn add_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1128493098)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 42, 112, 67, 67], OperandSize::Dword)
}

fn add_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1306063926)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 54, 244, 216, 77], OperandSize::Qword)
}

fn add_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RAX)), operand2: Some(Literal32(494082635)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 5, 75, 26, 115, 29], OperandSize::Qword)
}

fn add_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 100], OperandSize::Word)
}

fn add_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 4, 109], OperandSize::Word)
}

fn add_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 195, 5], OperandSize::Dword)
}

fn add_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 4, 241, 47], OperandSize::Dword)
}

fn add_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 116], OperandSize::Qword)
}

fn add_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 629383789, Some(OperandSize::Byte), None)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 143, 109, 162, 131, 37, 51], OperandSize::Qword)
}

fn add_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 195, 97], OperandSize::Qword)
}

fn add_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 574926098, Some(OperandSize::Byte), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 153, 18, 173, 68, 34, 43], OperandSize::Qword)
}

fn add_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Literal16(18593)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 194, 161, 72], OperandSize::Word)
}

fn add_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 21588, Some(OperandSize::Word), None)), operand2: Some(Literal16(13301)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 132, 84, 84, 245, 51], OperandSize::Word)
}

fn add_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Literal16(24243)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 199, 179, 94], OperandSize::Dword)
}

fn add_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(EAX, 1566114336, Some(OperandSize::Word), None)), operand2: Some(Literal16(15092)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 128, 32, 2, 89, 93, 244, 58], OperandSize::Dword)
}

fn add_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Literal16(12298)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 194, 10, 48], OperandSize::Qword)
}

fn add_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 962747274, Some(OperandSize::Word), None)), operand2: Some(Literal16(991)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 4, 133, 138, 91, 98, 57, 223, 3], OperandSize::Qword)
}

fn add_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1310843054)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 195, 174, 224, 33, 78], OperandSize::Word)
}

fn add_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 22, Some(OperandSize::Dword), None)), operand2: Some(Literal32(994789698)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 68, 22, 66, 73, 75, 59], OperandSize::Word)
}

fn add_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(Literal32(630370969)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 193, 153, 178, 146, 37], OperandSize::Dword)
}

fn add_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal32(323761652)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 4, 242, 244, 53, 76, 19], OperandSize::Dword)
}

fn add_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1613825950)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 199, 158, 7, 49, 96], OperandSize::Qword)
}

fn add_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 210319424, Some(OperandSize::Dword), None)), operand2: Some(Literal32(729993786)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 132, 158, 64, 56, 137, 12, 58, 210, 130, 43], OperandSize::Qword)
}

fn add_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RSP)), operand2: Some(Literal32(134145112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 196, 88, 228, 254, 7], OperandSize::Qword)
}

fn add_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RCX, 689133230, Some(OperandSize::Qword), None)), operand2: Some(Literal32(661192379)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 129, 174, 86, 19, 41, 187, 254, 104, 39], OperandSize::Qword)
}

fn add_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 193, 20], OperandSize::Word)
}

fn add_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 5, 115], OperandSize::Word)
}

fn add_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 198, 115], OperandSize::Dword)
}

fn add_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1236506348, Some(OperandSize::Word), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 4, 93, 236, 150, 179, 73, 81], OperandSize::Dword)
}

fn add_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 193, 41], OperandSize::Qword)
}

fn add_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RCX, 1214858947, Some(OperandSize::Word), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 129, 195, 70, 105, 72, 77], OperandSize::Qword)
}

fn add_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 198, 19], OperandSize::Word)
}

fn add_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 7, 58], OperandSize::Word)
}

fn add_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 198, 9], OperandSize::Dword)
}

fn add_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(EAX, Four, 328988062, Some(OperandSize::Dword), None)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 4, 133, 158, 245, 155, 19, 111], OperandSize::Dword)
}

fn add_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 193, 53], OperandSize::Qword)
}

fn add_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RDX, 96349, Some(OperandSize::Dword), None)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 130, 93, 120, 1, 0, 39], OperandSize::Qword)
}

fn add_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDI)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 199, 93], OperandSize::Qword)
}

fn add_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1778030502, Some(OperandSize::Qword), None)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 132, 139, 166, 151, 250, 105, 60], OperandSize::Qword)
}

