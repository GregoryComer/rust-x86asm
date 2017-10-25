use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 203], OperandSize::Dword)
}

fn vaesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 27], OperandSize::Dword)
}

fn vaesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 238], OperandSize::Qword)
}

fn vaesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 52, 136], OperandSize::Qword)
}

