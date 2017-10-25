use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 254], OperandSize::Word)
}

fn bt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 21205, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 176, 213, 82], OperandSize::Word)
}

fn bt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 255], OperandSize::Dword)
}

fn bt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(EAX, 730245522, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 168, 146, 169, 134, 43], OperandSize::Dword)
}

fn bt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 246], OperandSize::Qword)
}

fn bt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(RCX, 1345559301, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 145, 5, 155, 51, 80], OperandSize::Qword)
}

fn bt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 249], OperandSize::Word)
}

fn bt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 22, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 97, 22], OperandSize::Word)
}

fn bt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 250], OperandSize::Dword)
}

fn bt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 49], OperandSize::Dword)
}

fn bt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 253], OperandSize::Qword)
}

fn bt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 20, 251], OperandSize::Qword)
}

fn bt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 207], OperandSize::Qword)
}

fn bt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(RDI, Two, 997101862, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 60, 125, 38, 145, 110, 59], OperandSize::Qword)
}

fn bt_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 226, 57], OperandSize::Word)
}

fn bt_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 35, 48], OperandSize::Word)
}

fn bt_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BP)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 229, 36], OperandSize::Dword)
}

fn bt_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 36, 71, 82], OperandSize::Dword)
}

fn bt_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 226, 53], OperandSize::Qword)
}

fn bt_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 39, 7], OperandSize::Qword)
}

fn bt_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 226, 90], OperandSize::Word)
}

fn bt_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 29503, Some(OperandSize::Dword), None)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 163, 63, 115, 102], OperandSize::Word)
}

fn bt_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDX)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 226, 92], OperandSize::Dword)
}

fn bt_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 215725030, Some(OperandSize::Dword), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 164, 154, 230, 179, 219, 12, 77], OperandSize::Dword)
}

fn bt_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EBP)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 229, 86], OperandSize::Qword)
}

fn bt_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 684478139, Some(OperandSize::Dword), None)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 164, 187, 187, 78, 204, 40, 28], OperandSize::Qword)
}

fn bt_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RSI)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 230, 94], OperandSize::Qword)
}

fn bt_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(RSI, 1178742280, Some(OperandSize::Qword), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 166, 8, 46, 66, 70, 24], OperandSize::Qword)
}

