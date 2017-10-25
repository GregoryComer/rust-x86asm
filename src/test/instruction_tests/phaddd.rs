use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 254], OperandSize::Dword)
}

fn phaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 57], OperandSize::Dword)
}

fn phaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 221], OperandSize::Qword)
}

fn phaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 20, 251], OperandSize::Qword)
}

fn phaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 237], OperandSize::Dword)
}

fn phaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1128571496, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 4, 157, 104, 162, 68, 67], OperandSize::Dword)
}

fn phaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 225], OperandSize::Qword)
}

fn phaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 409991506, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 12, 253, 82, 249, 111, 24], OperandSize::Qword)
}

