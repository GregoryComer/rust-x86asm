use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 195, 29], OperandSize::Dword)
}

fn cmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 696649544, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 44, 93, 72, 7, 134, 41, 121], OperandSize::Dword)
}

fn cmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 241, 12], OperandSize::Qword)
}

fn cmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1122707249, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 52, 213, 49, 39, 235, 66, 0], OperandSize::Qword)
}

