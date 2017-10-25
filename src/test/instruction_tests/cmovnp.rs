use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 241], OperandSize::Word)
}

fn cmovnp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 152, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 140, 152, 0], OperandSize::Word)
}

fn cmovnp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 219], OperandSize::Dword)
}

fn cmovnp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 212123832, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 44, 125, 184, 192, 164, 12], OperandSize::Dword)
}

fn cmovnp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 247], OperandSize::Qword)
}

fn cmovnp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 214780736, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 12, 197, 64, 75, 205, 12], OperandSize::Qword)
}

fn cmovnp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 247], OperandSize::Word)
}

fn cmovnp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 25904, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 168, 48, 101], OperandSize::Word)
}

fn cmovnp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 251], OperandSize::Dword)
}

fn cmovnp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EBX, 1560681813, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 171, 85, 29, 6, 93], OperandSize::Dword)
}

fn cmovnp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 252], OperandSize::Qword)
}

fn cmovnp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1875344951, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 28, 141, 55, 126, 199, 111], OperandSize::Qword)
}

fn cmovnp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 222], OperandSize::Qword)
}

fn cmovnp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 36, 78], OperandSize::Qword)
}

