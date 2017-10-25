use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 206], OperandSize::Dword)
}

fn paddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 909116417, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 140, 176, 1, 4, 48, 54], OperandSize::Dword)
}

fn paddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 203], OperandSize::Qword)
}

fn paddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1332844986, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 132, 113, 186, 153, 113, 79], OperandSize::Qword)
}

fn paddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 214], OperandSize::Dword)
}

fn paddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 0], OperandSize::Dword)
}

fn paddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 201], OperandSize::Qword)
}

fn paddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 16], OperandSize::Qword)
}

