use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 205], OperandSize::Dword)
}

fn cvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 26], OperandSize::Dword)
}

fn cvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 233], OperandSize::Qword)
}

fn cvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 39], OperandSize::Qword)
}

fn cvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 211], OperandSize::Qword)
}

fn cvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 102986682, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 12, 149, 186, 115, 35, 6], OperandSize::Qword)
}

