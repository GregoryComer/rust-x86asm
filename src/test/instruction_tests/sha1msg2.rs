use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha1msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 254], OperandSize::Dword)
}

fn sha1msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1528022150, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 4, 181, 134, 196, 19, 91], OperandSize::Dword)
}

fn sha1msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 208], OperandSize::Qword)
}

fn sha1msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1160327152, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 20, 253, 240, 47, 41, 69], OperandSize::Qword)
}

