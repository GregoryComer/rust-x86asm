use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 222], OperandSize::Dword)
}

fn cvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 108698665, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 156, 192, 41, 156, 122, 6], OperandSize::Dword)
}

fn cvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 244], OperandSize::Qword)
}

fn cvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1713558587, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 44, 188, 80, 59, 212, 34, 102], OperandSize::Qword)
}

fn cvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 200], OperandSize::Qword)
}

fn cvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1489502932, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 44, 28, 149, 212, 2, 200, 88], OperandSize::Qword)
}

