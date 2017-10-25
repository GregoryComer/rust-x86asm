use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 56, 121, 238], OperandSize::Dword)
}

fn vcvtsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 319795005, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 188, 243, 61, 175, 15, 19], OperandSize::Dword)
}

fn vcvtsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 120, 121, 218], OperandSize::Qword)
}

fn vcvtsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 465996539, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 148, 71, 251, 138, 198, 27], OperandSize::Qword)
}

fn vcvtsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 255, 88, 121, 210], OperandSize::Qword)
}

fn vcvtsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 121, 57], OperandSize::Qword)
}

