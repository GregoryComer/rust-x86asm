use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 219, 24], OperandSize::Dword)
}

fn psrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 222, 34], OperandSize::Qword)
}

