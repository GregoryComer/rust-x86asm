use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 238], OperandSize::Word)
}

fn cvtps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Indirect(DI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 37], OperandSize::Word)
}

fn cvtps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 254], OperandSize::Dword)
}

fn cvtps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1991781257, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 52, 69, 137, 43, 184, 118], OperandSize::Dword)
}

fn cvtps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 208], OperandSize::Qword)
}

fn cvtps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RCX, 175645611, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 145, 171, 35, 120, 10], OperandSize::Qword)
}

