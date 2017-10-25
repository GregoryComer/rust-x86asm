use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 239], OperandSize::Word)
}

fn cmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 241, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 171, 241, 0], OperandSize::Word)
}

fn cmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 238], OperandSize::Dword)
}

fn cmovnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(ECX, 136817619, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 161, 211, 171, 39, 8], OperandSize::Dword)
}

fn cmovnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 234], OperandSize::Qword)
}

fn cmovnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RCX, 494262662, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 137, 134, 217, 117, 29], OperandSize::Qword)
}

fn cmovnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 253], OperandSize::Word)
}

fn cmovnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDI)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 60], OperandSize::Word)
}

fn cmovnbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 241], OperandSize::Dword)
}

fn cmovnbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EDX, 357354480, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 138, 240, 203, 76, 21], OperandSize::Dword)
}

fn cmovnbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 242], OperandSize::Qword)
}

fn cmovnbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 30], OperandSize::Qword)
}

fn cmovnbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 215], OperandSize::Qword)
}

fn cmovnbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1469068055, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 36, 181, 23, 51, 144, 87], OperandSize::Qword)
}

