use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 214], OperandSize::Word)
}

fn cmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(BX, 202, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 167, 202, 0], OperandSize::Word)
}

fn cmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 210], OperandSize::Dword)
}

fn cmovbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(ESI, 1479352243, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 166, 179, 31, 45, 88], OperandSize::Dword)
}

fn cmovbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 237], OperandSize::Qword)
}

fn cmovbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 44, 78], OperandSize::Qword)
}

fn cmovbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 221], OperandSize::Word)
}

fn cmovbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(BP, 2819, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 166, 3, 11], OperandSize::Word)
}

fn cmovbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 204], OperandSize::Dword)
}

fn cmovbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EBX, 1757854548, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 155, 84, 187, 198, 104], OperandSize::Dword)
}

fn cmovbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 215], OperandSize::Qword)
}

fn cmovbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1574656368, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 52, 189, 112, 89, 219, 93], OperandSize::Qword)
}

fn cmovbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 220], OperandSize::Qword)
}

fn cmovbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 281159885, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 172, 87, 205, 40, 194, 16], OperandSize::Qword)
}

