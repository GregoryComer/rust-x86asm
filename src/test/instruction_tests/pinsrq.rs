use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(RCX)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 225, 119], OperandSize::Qword)
}

fn pinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 36, 254, 87], OperandSize::Qword)
}

