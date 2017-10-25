use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 237], OperandSize::Dword)
}

fn vcvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 56], OperandSize::Dword)
}

fn vcvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 237], OperandSize::Qword)
}

fn vcvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 701904266, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 28, 205, 138, 53, 214, 41], OperandSize::Qword)
}

fn vcvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 205], OperandSize::Qword)
}

fn vcvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RCX, 1289628663, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 137, 247, 43, 222, 76], OperandSize::Qword)
}

fn vcvttss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 44, 220], OperandSize::Dword)
}

fn vcvttss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 55], OperandSize::Dword)
}

fn vcvttss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 126, 24, 44, 252], OperandSize::Qword)
}

fn vcvttss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RAX, 400817939, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 176, 19, 255, 227, 23], OperandSize::Qword)
}

fn vcvttss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 254, 24, 44, 221], OperandSize::Qword)
}

fn vcvttss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1118456462, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 52, 221, 142, 74, 170, 66], OperandSize::Qword)
}

