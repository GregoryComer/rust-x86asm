use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 229], OperandSize::Dword)
}

fn cvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 678321109, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 4, 221, 213, 91, 110, 40], OperandSize::Dword)
}

fn cvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 195], OperandSize::Qword)
}

fn cvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 12, 144], OperandSize::Qword)
}

fn cvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 251], OperandSize::Qword)
}

fn cvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 20, 145], OperandSize::Qword)
}

