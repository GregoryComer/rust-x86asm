use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 229], OperandSize::Dword)
}

fn vcvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 41], OperandSize::Dword)
}

fn vcvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 229], OperandSize::Qword)
}

fn vcvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 323700851, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 60, 213, 115, 72, 75, 19], OperandSize::Qword)
}

fn vcvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 243], OperandSize::Qword)
}

fn vcvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RDI, 897747125, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 175, 181, 136, 130, 53], OperandSize::Qword)
}

fn vcvtsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 24, 45, 243], OperandSize::Dword)
}

fn vcvtsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 961343277, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 188, 121, 45, 239, 76, 57], OperandSize::Dword)
}

fn vcvtsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 127, 88, 45, 231], OperandSize::Qword)
}

fn vcvtsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 22], OperandSize::Qword)
}

fn vcvtsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 255, 88, 45, 249], OperandSize::Qword)
}

fn vcvtsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 49], OperandSize::Qword)
}

