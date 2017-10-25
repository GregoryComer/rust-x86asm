use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn roundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 246, 20], OperandSize::Dword)
}

fn roundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1599820137, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 12, 245, 105, 81, 91, 95, 17], OperandSize::Dword)
}

fn roundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 210, 102], OperandSize::Qword)
}

fn roundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 8, 50, 5], OperandSize::Qword)
}

