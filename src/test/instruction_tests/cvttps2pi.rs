use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvttps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 235], OperandSize::Word)
}

fn cvttps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 58], OperandSize::Word)
}

fn cvttps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 213], OperandSize::Dword)
}

fn cvttps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(ECX, 1875535023, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 161, 175, 100, 202, 111], OperandSize::Dword)
}

fn cvttps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 251], OperandSize::Qword)
}

fn cvttps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2PI, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 44, 12, 208], OperandSize::Qword)
}

