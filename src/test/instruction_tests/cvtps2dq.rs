use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 198], OperandSize::Dword)
}

fn cvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 12, 217], OperandSize::Dword)
}

fn cvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 230], OperandSize::Qword)
}

fn cvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 23], OperandSize::Qword)
}

