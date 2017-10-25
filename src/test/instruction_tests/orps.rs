use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn orps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 219], OperandSize::Dword)
}

fn orps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 52, 73], OperandSize::Dword)
}

fn orps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 220], OperandSize::Qword)
}

fn orps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 782985951, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 180, 127, 223, 106, 171, 46], OperandSize::Qword)
}

