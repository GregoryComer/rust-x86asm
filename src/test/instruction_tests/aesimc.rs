use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 242], OperandSize::Dword)
}

fn aesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1388342617, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 180, 190, 89, 109, 192, 82], OperandSize::Dword)
}

fn aesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 218], OperandSize::Qword)
}

fn aesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 20, 120], OperandSize::Qword)
}

