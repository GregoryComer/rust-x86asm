use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 248], OperandSize::Dword)
}

fn vcvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 110028734, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 12, 125, 190, 231, 142, 6], OperandSize::Dword)
}

fn vcvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 200], OperandSize::Qword)
}

fn vcvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 172200090, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 60, 181, 154, 144, 67, 10], OperandSize::Qword)
}

fn vcvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 214], OperandSize::Qword)
}

fn vcvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1420994988, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 12, 77, 172, 169, 178, 84], OperandSize::Qword)
}

fn vcvttsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 44, 255], OperandSize::Dword)
}

fn vcvttsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 36, 138], OperandSize::Dword)
}

fn vcvttsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 127, 24, 44, 202], OperandSize::Qword)
}

fn vcvttsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 176748052, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 44, 188, 145, 20, 246, 136, 10], OperandSize::Qword)
}

fn vcvttsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 255, 24, 44, 201], OperandSize::Qword)
}

fn vcvttsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 44, 19], OperandSize::Qword)
}

