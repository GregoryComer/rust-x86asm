use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha1rnds4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 192, 74], OperandSize::Dword)
}

fn sha1rnds4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 306469397, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 134, 21, 90, 68, 18, 75], OperandSize::Dword)
}

fn sha1rnds4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 212, 100], OperandSize::Qword)
}

fn sha1rnds4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1RNDS4, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 1722974936, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 204, 138, 216, 130, 178, 102, 62], OperandSize::Qword)
}

