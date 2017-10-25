use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 196, 118], OperandSize::Dword)
}

fn cmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1218737498, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 28, 197, 90, 117, 164, 72, 20], OperandSize::Dword)
}

fn cmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 195, 31], OperandSize::Qword)
}

fn cmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 20, 250, 37], OperandSize::Qword)
}

