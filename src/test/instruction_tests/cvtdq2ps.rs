use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 198], OperandSize::Dword)
}

fn cvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 52, 248], OperandSize::Dword)
}

fn cvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 252], OperandSize::Qword)
}

fn cvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 7], OperandSize::Qword)
}

