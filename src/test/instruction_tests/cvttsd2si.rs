use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 244], OperandSize::Dword)
}

fn cvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(ECX, 1172962118, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 185, 70, 251, 233, 69], OperandSize::Dword)
}

fn cvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 207], OperandSize::Qword)
}

fn cvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 60, 135], OperandSize::Qword)
}

fn cvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 239], OperandSize::Qword)
}

fn cvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 28, 145], OperandSize::Qword)
}

