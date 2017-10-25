use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 251, 33], OperandSize::Dword)
}

fn cmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 442416669, Some(OperandSize::Qword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 140, 80, 29, 190, 94, 26, 23], OperandSize::Dword)
}

fn cmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 218, 49], OperandSize::Qword)
}

fn cmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 827163964, Some(OperandSize::Qword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 194, 180, 122, 60, 133, 77, 49, 43], OperandSize::Qword)
}

