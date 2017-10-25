use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 227], OperandSize::Dword)
}

fn cvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 36, 120], OperandSize::Dword)
}

fn cvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 196], OperandSize::Qword)
}

fn cvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 1987521547, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 90, 143, 11, 44, 119, 118], OperandSize::Qword)
}

