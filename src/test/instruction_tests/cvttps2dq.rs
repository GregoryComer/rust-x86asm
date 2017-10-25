use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 203], OperandSize::Dword)
}

fn cvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 461874170, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 60, 133, 250, 163, 135, 27], OperandSize::Dword)
}

fn cvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 200], OperandSize::Qword)
}

fn cvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDX, 727675980, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 186, 76, 116, 95, 43], OperandSize::Qword)
}

