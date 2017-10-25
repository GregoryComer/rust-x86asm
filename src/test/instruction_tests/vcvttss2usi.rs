use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 120, 206], OperandSize::Dword)
}

fn vcvttss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 895283585, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 28, 117, 129, 241, 92, 53], OperandSize::Dword)
}

fn vcvttss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 126, 24, 120, 247], OperandSize::Qword)
}

fn vcvttss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 86939615, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 120, 156, 74, 223, 151, 46, 5], OperandSize::Qword)
}

fn vcvttss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 254, 24, 120, 225], OperandSize::Qword)
}

fn vcvttss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2USI, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDX, 1396782558, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 120, 138, 222, 53, 65, 83], OperandSize::Qword)
}

