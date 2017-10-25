use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lsl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 247], OperandSize::Word)
}

fn lsl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(CX)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 12], OperandSize::Word)
}

fn lsl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 253], OperandSize::Dword)
}

fn lsl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(ECX, 265794957, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 185, 141, 181, 215, 15], OperandSize::Dword)
}

fn lsl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 219], OperandSize::Qword)
}

fn lsl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 20, 177], OperandSize::Qword)
}

fn lsl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 205], OperandSize::Word)
}

fn lsl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 6130, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 3, 170, 242, 23], OperandSize::Word)
}

fn lsl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 249], OperandSize::Dword)
}

fn lsl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 28, 66], OperandSize::Dword)
}

fn lsl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 235], OperandSize::Qword)
}

fn lsl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1859518299, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 3, 36, 213, 91, 255, 213, 110], OperandSize::Qword)
}

fn lsl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RSP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 227], OperandSize::Qword)
}

fn lsl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LSL, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 3, 28, 131], OperandSize::Qword)
}

