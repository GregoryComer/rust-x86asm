use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fnstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectDisplaced(SI, 22496, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 188, 224, 87], OperandSize::Word)
}

fn fnstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(EDX, Two, 1415558018, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 85, 130, 179, 95, 84], OperandSize::Dword)
}

fn fnstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(IndirectScaledDisplaced(RBX, Two, 2139074752, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 60, 93, 192, 176, 127, 127], OperandSize::Qword)
}

fn fnstsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Direct(AX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 224], OperandSize::Word)
}

fn fnstsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Direct(AX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 224], OperandSize::Dword)
}

fn fnstsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTSW, operand1: Some(Direct(AX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 224], OperandSize::Qword)
}

